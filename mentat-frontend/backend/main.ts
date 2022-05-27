import { nanoid } from '@reduxjs/toolkit';
import express from 'express';
import fetch from 'node-fetch';
import path from 'path';
import pkg from '../package.json';
import { RosettaError } from '../src/features/rosetta/models';
import db from './db';
import { IServers } from './models';
import { getNetworksFromUrl, getValidUrl } from './scanner';
import { MentatStatus } from './types';

let servers: IServers = db.read<IServers>() ?? {};

const app = express();

app.use(express.json());
app.use(express.static('dist'));

const getStatus = (): MentatStatus => ({ version: pkg.version, servers });

// get servers and mentat version
app.get('/api/v1/mentat', (_req, res) => {
  res.status(200).json(getStatus());
});

// data injection via js
app.get('/api/v1/servers.js', (req, res) => {
  res.status(200).send(`
    window.MENTAT = ${JSON.stringify(getStatus())};
  `);
});

// add a new server to the servers database
app.post('/api/v1/servers/:id?', async (req, res) => {
  const { url, name, force } = req.body;
  const id = req.params.id || nanoid();

  if (!url || !name)
    return res.status(422).json({
      code: -1,
      message: `Missing ${!url ? 'url' : 'name'} in body`,
      retriable: false,
    } as RosettaError);

  if (id in servers)
    return res.status(409).json({
      code: -1,
      message: `Server id already in use`,
      retriable: false,
    } as RosettaError);

  try {
    const validUrl = getValidUrl(url);
    if (!validUrl)
      return res.status(422).json({
        code: -1,
        message: 'Invalid uri (expected http://ip:port)',
        retriable: false,
      } as RosettaError);

    // if we're not forcing this, check the network url
    if (!force) {
      const resp = await getNetworksFromUrl(validUrl);
      if (!resp)
        return res.status(422).json({
          code: -1,
          message: 'Url did not respond per Rosetta API spec',
          retriable: false,
        } as RosettaError);

      console.info(
        '[info] adding server',
        id,
        'from',
        url,
        'with networks',
        resp
      );
    }

    servers = { ...servers, [id]: { url: validUrl.origin, name } };
    db.write(servers);
    res.status(200).json(getStatus());
  } catch (err) {
    res.status(500).json({
      code: -1,
      message: `Error getting networks from provided`,
      retriable: false,
    } as RosettaError);
  }
});

// remove an existing server from the servers database
app.delete('/api/v1/servers/:id', async (req, res) => {
  const { id } = req.params;

  if (!(id in servers))
    return res.status(404).json({
      code: -1,
      message: `Server id not found`,
      retriable: false,
    } as RosettaError);

  // remove an entry from the servers
  // eslint-disable-next-line
  const { [id]: _unused, ...rest } = servers;
  servers = rest;
  db.write(servers);

  res.status(200).json(getStatus());
});

// scan a url for rosetta networks
app.post('/api/v1/scan', async (req, res) => {
  try {
    const { url } = req.body;
    if (!url) return;

    const ids = await getNetworksFromUrl(url);
    res.status(200).json(ids);
  } catch (err) {
    console.error('error scanning rosetta endpoint', err);
    res.status(500).json({
      code: -1,
      message: 'Error scanning endpoint',
      retriable: false,
    } as RosettaError);
  }
});

// proxy rosetta requests
app.all('/api/rosetta/:id/*', async (req, res) => {
  const { id } = req.params;
  if (!(id in servers)) {
    res.status(404).send({
      code: -1,
      message: 'Endpoint not found',
      retriable: false,
      details: {
        message: `Backend could not find an endpoint with id "${id}"`,
      },
    } as RosettaError);
    return;
  }

  const target = servers[id].url;

  // remove the prefix in the proxied request url
  const url = req.url.replace(new RegExp(`^/api/rosetta/${id}/`), '/');

  try {
    // galaxy brain reverse proxy
    const resp = await fetch(target + url, {
      method: req.method,
      headers: req.headers as Record<string, string>,
      body: JSON.stringify(req.body),
    });

    const body = await resp.text();

    res.status(resp.status).send(body);
  } catch (err) {
    res.status(500).send({
      code: -1,
      message: 'Error making proxy request',
      retriable: false,
      details: {
        message: err?.message ?? err.toString(),
      },
    } as RosettaError);
  }
});

// server the index file instead of a 404
app.use((_req, res, _next) => {
  res.status(404).sendFile(path.join(__dirname, 'index.html'));
});

app.listen(Number(process.env.PORT ?? 3000));

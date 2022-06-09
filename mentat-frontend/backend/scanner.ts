import fetch from 'node-fetch';
import {
  NetworkIdentifier,
  RosettaError,
} from '../src/features/rosetta/models';

export function getValidUrl(str: string) {
  try {
    const url = new URL(str);

    // ensure this url is a valid http url with no path, search, username, or hash
    if (
      (url.protocol !== 'http:' && url.protocol !== 'https:') ||
      url.search ||
      url.username ||
      url.hash
    )
      return null;

    return url;
  } catch (err) {
    return null;
  }
}

/** attempt to get network identifiers from a rosetta api endpoint url */
export async function getNetworksFromUrl(
  endpointUrl: string | URL
): Promise<NetworkIdentifier[] | RosettaError> {
  try {
    const url =
      typeof endpointUrl === 'string' ? getValidUrl(endpointUrl) : endpointUrl;

    // ensure this url is a valid http url with no path, search, username, or hash
    if (!url) {
      return {
        code: -1,
        message: 'Invalid uri (expected http://ip:port)',
        retriable: false,
      };
    }

    const resp = await fetch(url.origin + '/network/list', {
      method: 'POST',
      body: '{}',
      headers: {
        'content-type': 'application/json',
      },
    });

    const blob = await resp.json();

    if (
      !blob['network_identifiers'] ||
      !Array.isArray(blob['network_identifiers'])
    )
      return {
        code: -1,
        message: 'Invalid response (expected { network_identifiers: [] })',
        retriable: false,
      };

    return blob['network_identifiers'];
  } catch (err) {
    return {
      code: -1,
      message: 'Error fetching network JSON',
      retriable: false,
      details: err?.stack,
    };
  }
}

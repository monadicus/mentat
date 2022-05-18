import { AppDispatch } from './../../store';
import { useEffect, useMemo, useState } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { useParams } from 'react-router';
import { selectNetworkIdentifier } from './selectors';
import { addError } from '../errors/reducer';
import { RosettaError } from './models';

export const useNetId = () => useSelector(selectNetworkIdentifier);

export const useLinkRoute = (...path: string[]) => {
  const { endpoint } = useParams();
  return `/${endpoint}/${path.map(s => encodeURIComponent(s)).join('/')}`;
};

/** Get the endpoint url from route params */
export function useEndpointUrl() {
  const { endpoint } = useParams();
  if (!endpoint) return null;

  const host = (() => {
    // ~ redirects to localhost
    if (endpoint === '~') return location.hostname + ':8080';

    // allow :port
    if (endpoint.match(/\d+/)) return '127.0.0.1:' + endpoint;

    // default to the value of the host
    return endpoint;
  })();

  return `${location.protocol}//${host}`;
}

export type ApiState = 'init' | 'loading' | 'ok' | 'error';

const empty = {};

export function useErrorHandling<T extends Record<string, unknown>>(
  [status, resp]: [ApiState, null | T | RosettaError],
  activity?: string
): [ApiState, null | T] {
  const dispatch: AppDispatch = useDispatch();
  return useMemo(() => {
    // probably an error
    if (
      status === 'ok' &&
      resp &&
      'code' in resp &&
      typeof resp.code === 'number' &&
      'message' in resp &&
      typeof resp.message === 'string' &&
      'retriable' in resp &&
      typeof resp.retriable === 'boolean'
    ) {
      dispatch(addError({ ...resp } as RosettaError, activity));
      return ['error', null];
    }

    // probably not an error?
    return [status, resp as T];
  }, [status, resp, dispatch, activity]);
}

/** make a request to the rosetta-api from the route param */
export function useApiUnhandled<
  T extends Record<string, unknown> = Record<string, unknown>
>(
  path: string,
  requestBody: Record<string, unknown> = empty,
  opts?: RequestInit
): [ApiState, null | T | RosettaError] {
  const [status, setStatus] = useState<ApiState>('init');
  const [response, setResponse] = useState<null | T>(null);

  const url = useEndpointUrl();
  useEffect(() => {
    let unmount = false;
    setStatus('loading');
    setResponse(null);

    (async () => {
      try {
        // make the request
        const resp = await fetch(url + path, {
          method: 'POST',
          body: JSON.stringify(requestBody),
          ...(opts ?? {}),
          headers: {
            'content-type': 'application/json',
            ...(opts?.headers ?? {}),
          },
        });

        if (unmount) return;
        // read body as text
        const body = await resp.text();

        // parse body as json
        try {
          const blob: T = JSON.parse(body);
          if (unmount) return;
          setStatus('ok');
          setResponse(blob);
        } catch (err) {
          if (unmount) return;
          console.error('error parsing request body', path, body, err);
          setStatus('error');
        }
      } catch (err) {
        if (unmount) return;
        console.error('error making request', path, err);
        setStatus('error');
      }
    })();

    return () => {
      unmount = true;
    };
  }, [path, opts, url, requestBody]);

  return useMemo(() => [status, response], [status, response]);
}

export function useApi<
  T extends Record<string, unknown> = Record<string, unknown>
>(
  path: string,
  requestBody: Record<string, unknown> = empty,
  opts?: RequestInit
): [ApiState, null | T] {
  return useErrorHandling(
    useApiUnhandled(path, requestBody, opts),
    'API Req: ' + path
  );
}

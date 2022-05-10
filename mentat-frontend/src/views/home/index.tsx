import React from 'react';
import { useSelector } from 'react-redux';
import { BlockId } from '../../components/Blocks';
import { i18n, I18n } from '../../features/i18n/components';
import {
  selectNetworkOptions,
  selectNetworkStatus,
} from '../../features/rosetta/selectors';

export const Home = () => {
  const status = useSelector(selectNetworkStatus);
  const options = useSelector(selectNetworkOptions);

  return (
    <>
      <h2>
        <I18n name="navigation.home" />
      </h2>
      <br />
      {(!status || !options) && <I18n name="navigation.loading" />}
      {/* <pre>{JSON.stringify(resp, null, 2)}</pre> */}
      {status && (
        <>
          <h3>
            <I18n name="views.home.blocks_header" />
          </h3>
          <BlockId
            id={status.current_block_identifier}
            label={i18n('views.blocks.current_block_label')}
          />
          <BlockId
            id={status.genesis_block_identifier}
            label={i18n('views.blocks.genesis_block_label')}
          />
          <h3>
            <I18n name="views.home.peers_header" />
          </h3>
          <ul>
            {status.peers.map(p => (
              <li key={p.peer_id}>{p.peer_id}</li>
            ))}
          </ul>
        </>
      )}
      {options && (
        <>
          <h3>
            <I18n name="views.home.options" />
          </h3>
          <pre>{JSON.stringify(options, null, 2)}</pre>
        </>
      )}
    </>
  );
};

import React from 'react';
import { useNetId } from '../../App';
import { BlockId } from '../../components/Blocks';
import { i18n, I18n } from '../../features/i18n/components';
import { useNetOptions, useNetStatus } from '../../features/rosetta/impl';

export const Home = () => {
  const netId = useNetId();
  const [status, resp] = useNetStatus(netId);
  const [optionsStatus, optionsResp] = useNetOptions(netId);

  return (
    <>
      <h2>
        <I18n name="navigation.home" />
      </h2>
      <br />
      {(status === 'loading' || optionsStatus === 'loading') && (
        <I18n name="navigation.loading" />
      )}
      {/* <pre>{JSON.stringify(resp, null, 2)}</pre> */}
      {resp && (
        <>
          <h3>
            <I18n name="views.home.blocks_header" />
          </h3>
          <BlockId
            id={resp.current_block_identifier}
            label={i18n('views.blocks.current_block_label')}
          />
          <BlockId
            id={resp.genesis_block_identifier}
            label={i18n('views.blocks.genesis_block_label')}
          />
          <h3>
            <I18n name="views.home.peers_header" />
          </h3>
          <ul>
            {resp.peers.map(p => (
              <li key={p.peer_id}>{p.peer_id}</li>
            ))}
          </ul>
        </>
      )}
      {optionsResp && (
        <>
          <h3>
            <I18n name="views.home.options" />
          </h3>
          <pre>{JSON.stringify(optionsResp, null, 2)}</pre>
        </>
      )}
    </>
  );
};

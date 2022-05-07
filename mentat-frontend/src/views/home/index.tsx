import React from 'react';
import { useNetId } from '../../App';
import { BlockId } from '../../components/Blocks';
import { i18n, I18n } from '../../features/i18n/components';
import { useNetStatus } from '../../features/rosetta/impl';

export const Home = () => {
  const netId = useNetId();
  const [status, resp] = useNetStatus(netId);

  return (
    <>
      <I18n name="navigation.home" />
      <br />
      {status === 'loading' && <I18n name="navigation.loading" />}
      <pre>{JSON.stringify(resp, null, 2)}</pre>
      {resp && (
        <>
          <BlockId
            id={resp.current_block_identifier}
            label={i18n('views.blocks.current_block_label')}
          />
          <BlockId
            id={resp.genesis_block_identifier}
            label={i18n('views.blocks.genesis_block_label')}
          />
        </>
      )}
    </>
  );
};

import React, { useMemo } from 'react';
import { useParams } from 'react-router';
import { NavLink } from 'react-router-dom';
import { useNetId } from '../../App';
import { BlockId } from '../../components/Blocks';
import { I18n, i18n } from '../../features/i18n/components';
import { useApi } from '../../features/rosetta/hooks';
import { BlockResponse } from '../../features/rosetta/models';
import { BlockViewStyle } from './style';

export const BlockView = () => {
  const { index, hash, endpoint } = useParams();

  const network_identifier = useNetId();
  const block_identifier = useMemo(
    () => (hash ? { hash } : { index }),
    [hash, index]
  );

  const [status, resp] = useApi<BlockResponse>(
    '/block',
    useMemo(
      () => ({
        network_identifier,
        block_identifier,
      }),
      [network_identifier, block_identifier]
    )
  );

  const [, nextResp] = useApi<BlockResponse>(
    '/block',
    useMemo(
      () => ({
        network_identifier,
        block_identifier: resp
          ? { index: resp.block.block_identifier.index + 1 }
          : { index: 0 },
      }),
      [network_identifier, resp]
    )
  );
  console.debug('[debug]', nextResp);

  return (
    <BlockViewStyle>
      <div>loading: {status}</div>
      {resp && (
        <>
          {resp.block.parent_block_identifier.index !==
            resp.block.block_identifier.index && (
            <BlockId
              id={resp.block.parent_block_identifier}
              label={i18n('views.block.parent_block_label')}
            />
          )}
          <BlockId
            noLink
            id={resp.block.block_identifier}
            label={i18n('views.block.current_block_label')}
          />
          {nextResp?.block && (
            <BlockId
              id={nextResp.block.block_identifier}
              label={i18n('views.block.next_block_label')}
            />
          )}
          <h3>
            <I18n name="views.block.meta_header" />
          </h3>
          <table>
            <tbody>
              <tr>
                <td>
                  <b>
                    <I18n name="views.block.meta_timestamp_label" />
                  </b>
                </td>
                <td>{new Date(resp.block.timestamp).toString()}</td>
              </tr>
              <tr>
                <td>
                  <b>
                    <I18n name="views.block.meta_transactions_label" />
                  </b>
                </td>
                <td>{resp.block.transactions.length}</td>
              </tr>
            </tbody>
          </table>
          <h3>
            <I18n name="views.block.transactions_header" />
          </h3>
          <ol>
            {resp.block.transactions.map(t => (
              <li key={t.transaction_identifier.hash}>
                <div>
                  <NavLink
                    to={`/${endpoint}/transactions/${t.transaction_identifier.hash}`}
                  >
                    {t.transaction_identifier.hash}
                  </NavLink>
                </div>
                <div className="transaction" style={{ marginLeft: 14 }}>
                  <ul>
                    {t.operations.map(
                      ({
                        operation_identifier,
                        metadata: _metadata,
                        ...rest
                      }) => (
                        <li key={operation_identifier.index}>
                          {'account' in rest && (
                            <NavLink
                              to={`/${endpoint}/accounts/${rest.account.address}`}
                            >
                              <I18n name="views.block.account_text" />
                            </NavLink>
                          )}
                          <pre>{JSON.stringify(rest, null, 2)}</pre>
                        </li>
                      )
                    )}
                  </ul>
                </div>
              </li>
            ))}
          </ol>
          {/* <pre>{JSON.stringify(resp, null, 2)}</pre> */}
        </>
      )}
    </BlockViewStyle>
  );
};

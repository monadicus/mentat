import React, { useMemo } from 'react';
import { FaStar } from 'react-icons/fa';
import { useSelector } from 'react-redux';
import { useParams } from 'react-router';
import { NavLink } from 'react-router-dom';
import { BlockId } from '../../components/Blocks';
import {
  selectAccountAlias,
  selectIsAccountFollowed,
} from '../../features/accounts/selectors';
import { I18n, i18n } from '../../features/i18n/components';
import { useApi, useLinkRoute, useNetId } from '../../features/rosetta/hooks';
import { BlockResponse, Operation } from '../../features/rosetta/models';
import { BlockViewStyle } from './style';

const TransactionOperation: React.FC<{ operation: Operation }> = ({
  operation: { operation_identifier, metadata: _metadata, account, ...rest },
}) => {
  const route = useLinkRoute('accounts', account?.address);
  const alias = useSelector(selectAccountAlias(account?.address));
  const followed = useSelector(selectIsAccountFollowed(account?.address));
  return (
    <li key={operation_identifier.index}>
      {account && (
        <NavLink to={route}>
          {alias || <I18n name="views.block.account_text" />}
          {followed && <FaStar />}
        </NavLink>
      )}
      <pre>{JSON.stringify(rest, null, 2)}</pre>
    </li>
  );
};

export const BlockView = () => {
  const { index, hash, endpoint } = useParams();

  const network_identifier = useNetId();
  const block_identifier = useMemo(
    () => (hash ? { hash } : { index }),
    [hash, index]
  );

  const [, currResp] = useApi<BlockResponse>(
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
        block_identifier: currResp
          ? { index: currResp.block.block_identifier.index + 1 }
          : { index: 0 },
      }),
      [network_identifier, currResp]
    )
  );

  return (
    <BlockViewStyle>
      {currResp && (
        <>
          {currResp.block.parent_block_identifier.index !==
            currResp.block.block_identifier.index && (
            <BlockId
              id={currResp.block.parent_block_identifier}
              label={i18n('views.block.parent_block_label')}
            />
          )}
          <BlockId
            noLink
            id={currResp.block.block_identifier}
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
                <td>{new Date(currResp.block.timestamp).toString()}</td>
              </tr>
              <tr>
                <td>
                  <b>
                    <I18n name="views.block.meta_transactions_label" />
                  </b>
                </td>
                <td>{currResp.block.transactions.length}</td>
              </tr>
            </tbody>
          </table>
          <h3>
            <I18n name="views.block.transactions_header" />
          </h3>
          <ol>
            {currResp.block.transactions.map(t => (
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
                    {t.operations.map(o => (
                      <TransactionOperation
                        key={o.operation_identifier.index}
                        operation={o}
                      />
                    ))}
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

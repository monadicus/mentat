import React from 'react';
import { NavLink } from 'react-router-dom';
import { I18n } from '../../features/i18n/components';
import { useLinkRoute } from '../../features/rosetta/hooks';
import { BlockIdentifier } from '../../features/rosetta/models';
import { BlockIdStyle } from './style';

export const BlockId: React.FC<{
  id: BlockIdentifier;
  label: string;
  noLink?: boolean;
}> = ({ id, label, noLink }) => {
  const route = useLinkRoute('blocks', 'hash', id.hash);
  return (
    <BlockIdStyle>
      <label>{noLink ? label : <NavLink to={route}>{label}</NavLink>}</label>
      <div>
        <I18n name="components.blocks.index_label" />
      </div>
      <div>{id.index}</div>
      <div>
        <I18n name="components.blocks.hash_label" />
      </div>
      <div className="block-hash" title={id.hash}>
        {id.hash}
      </div>
    </BlockIdStyle>
  );
};

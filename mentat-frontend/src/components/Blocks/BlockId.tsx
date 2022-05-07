import React from 'react';
import { I18n } from '../../features/i18n/components';
import { BlockIdentifier } from '../../features/rosetta/models';
import { BlockIdStyle } from './style';

export const BlockId: React.FC<{
  id: BlockIdentifier;
  label: string;
}> = ({ id, label }) => {
  return (
    <BlockIdStyle>
      <label>{label}</label>
      <span>
        <I18n name="components.blocks.index_label" />
      </span>
      <span>{id.index}</span>
      <span>
        <I18n name="components.blocks.hash_label" />
      </span>
      <span>{id.hash}</span>
    </BlockIdStyle>
  );
};

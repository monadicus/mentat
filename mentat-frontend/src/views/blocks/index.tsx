import React, { useEffect } from 'react';
import { useNavigate, useParams } from 'react-router';
import { useNetId } from '../../App';
import { I18n } from '../../features/i18n/components';
import { useNetStatus } from '../../features/rosetta/impl';

export const BlocksView = () => {
  const { endpoint } = useParams();
  const [, res] = useNetStatus(useNetId());
  const navigate = useNavigate();
  useEffect(() => {
    if (res)
      navigate(`/${endpoint}/blocks/hash/${res.current_block_identifier.hash}`);
  }, [endpoint, navigate, res]);
  return (
    <>
      <I18n name="views.blocks.redirect_note" />
    </>
  );
};

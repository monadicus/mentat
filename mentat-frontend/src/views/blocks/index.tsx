import React, { useEffect } from 'react';
import { useSelector } from 'react-redux';
import { useNavigate, useParams } from 'react-router';
import { I18n } from '../../features/i18n/components';
import { useNetId } from '../../features/rosetta/hooks';
import { useNetStatus } from '../../features/rosetta/impl';
import { selectNetworkStatus } from '../../features/rosetta/selectors';

export const BlocksView = () => {
  const { endpoint } = useParams();
  const status = useSelector(selectNetworkStatus);

  const navigate = useNavigate();
  useEffect(() => {
    if (status)
      navigate(
        `/${endpoint}/blocks/hash/${status.current_block_identifier.hash}`
      );
  }, [endpoint, navigate, status]);

  return (
    <>
      <I18n name="views.blocks.redirect_note" />
    </>
  );
};

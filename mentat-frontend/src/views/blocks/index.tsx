import React, { useEffect } from 'react';
import { useSelector } from 'react-redux';
import { useNavigate } from 'react-router';
import { I18n } from '../../features/i18n/components';
import { useLinkRoute } from '../../features/rosetta/hooks';
import { selectNetworkStatus } from '../../features/rosetta/selectors';

export const BlocksView = () => {
  const status = useSelector(selectNetworkStatus);
  const route = useLinkRoute(
    'blocks',
    'hash',
    status?.current_block_identifier.hash ?? ''
  );

  const navigate = useNavigate();
  useEffect(() => {
    if (status) navigate(route, { replace: true });
  }, [navigate, route, status]);

  return (
    <>
      <I18n name="views.blocks.redirect_note" />
    </>
  );
};

import React, { useCallback } from 'react';
import { FaExchangeAlt, FaHome, FaNetworkWired, FaUsers } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { i18n } from '../../features/i18n/components';
import { toggleOpen } from '../../features/sidenav/reducer';
import { selectSideNavOpen } from '../../features/sidenav/selectors';
import { AppDispatch } from '../../store';
import { SideNavItem } from './SideNavItem';
import { SideNavShade, SideNavStyle } from './style';

export const SideNav = () => {
  const open = useSelector(selectSideNavOpen);
  const dispatch = useDispatch<AppDispatch>();
  const toggleSideNav = useCallback(() => dispatch(toggleOpen()), [dispatch]);

  return (
    <>
      <SideNavShade open={open} onClick={toggleSideNav} />
      <SideNavStyle open={open}>
        <div className="items">
          <SideNavItem
            to="/"
            name={i18n('navigation.sidenav.home')}
            icon={FaHome}
          />
          <SideNavItem
            to="/networks"
            name={i18n('navigation.sidenav.networks')}
            icon={FaNetworkWired}
          />
          <SideNavItem
            to="/accounts"
            name={i18n('navigation.sidenav.accounts')}
            icon={FaUsers}
          />
          <SideNavItem
            to="/transactions"
            name={i18n('navigation.sidenav.transactions')}
            icon={FaExchangeAlt}
          />
        </div>
      </SideNavStyle>
    </>
  );
};

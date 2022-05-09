import React, { useCallback } from 'react';
import { FaCubes, FaExchangeAlt, FaHome, FaUsers } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { i18n } from '../../features/i18n/components';
import { useLinkRoute } from '../../features/rosetta/hooks';
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
            to={useLinkRoute('')}
            name={i18n('navigation.sidenav.home')}
            icon={FaHome}
          />
          {/* <SideNavItem
            to={`/${endpoint}/networks`}
            name={i18n('navigation.sidenav.networks')}
            icon={FaNetworkWired}
          /> */}
          <SideNavItem
            to={useLinkRoute('accounts')}
            name={i18n('navigation.sidenav.accounts')}
            icon={FaUsers}
          />
          <SideNavItem
            to={useLinkRoute('blocks')}
            name={i18n('navigation.sidenav.blocks')}
            icon={FaCubes}
          />
          <SideNavItem
            to={useLinkRoute('transactions')}
            name={i18n('navigation.sidenav.transactions')}
            icon={FaExchangeAlt}
          />
        </div>
      </SideNavStyle>
    </>
  );
};

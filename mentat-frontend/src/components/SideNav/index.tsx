import React, { useCallback } from 'react';
import { FaCubes, FaExchangeAlt, FaHome, FaUsers } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { useParams } from 'react-router';
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
  const { endpoint } = useParams();

  return (
    <>
      <SideNavShade open={open} onClick={toggleSideNav} />
      <SideNavStyle open={open}>
        <div className="items">
          <SideNavItem
            to={`/${endpoint}/`}
            name={i18n('navigation.sidenav.home')}
            icon={FaHome}
          />
          {/* <SideNavItem
            to={`/${endpoint}/networks`}
            name={i18n('navigation.sidenav.networks')}
            icon={FaNetworkWired}
          /> */}
          <SideNavItem
            to={`/${endpoint}/accounts`}
            name={i18n('navigation.sidenav.accounts')}
            icon={FaUsers}
          />
          <SideNavItem
            to={`/${endpoint}/blocks`}
            name={i18n('navigation.sidenav.blocks')}
            icon={FaCubes}
          />
          <SideNavItem
            to={`/${endpoint}/transactions`}
            name={i18n('navigation.sidenav.transactions')}
            icon={FaExchangeAlt}
          />
        </div>
      </SideNavStyle>
    </>
  );
};

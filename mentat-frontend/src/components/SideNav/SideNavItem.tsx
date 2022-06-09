import React, { useCallback } from 'react';
import { NavLink } from 'react-router-dom';
import { IconType } from 'react-icons/lib';
import { useDispatch } from 'react-redux';
import { AppDispatch } from '../../store';
import { setClosed } from '../../features/sidenav/reducer';

export const SideNavItem: React.FC<{
  to: string;
  name: string;
  icon: IconType;
}> = ({ to, name, icon: Icon }) => {
  const dispatch = useDispatch<AppDispatch>();
  const closeSideNav = useCallback(() => dispatch(setClosed()), [dispatch]);

  return (
    <NavLink
      to={to}
      className={({ isActive }) => 'nav-link ' + (isActive ? 'active' : '')}
      onClick={closeSideNav}
    >
      <>
        <span className="icon">
          <Icon />
        </span>
        <span className="label">{name}</span>
      </>
    </NavLink>
  );
};

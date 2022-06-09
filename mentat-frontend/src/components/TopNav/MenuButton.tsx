import React, { useCallback } from 'react';
import { useDispatch } from 'react-redux';
import styled from 'styled-components';
import media from 'styled-media-query';
import { toggleOpen } from '../../features/sidenav/reducer';
import { AppDispatch } from '../../store';
import { FaBars } from 'react-icons/fa';

export const MenuButtonStyle = styled.div`
  ${media.greaterThan('medium')`display: none;`}
  display: flex;
  align-items: center;
  margin: ${props => -props.theme.margin}px;
  margin-right: 0px;
  padding: ${props => props.theme.margin}px;
  cursor: pointer;
`;

export const MenuButton = () => {
  const dispatch = useDispatch<AppDispatch>();
  const toggleSideNav = useCallback(() => dispatch(toggleOpen()), [dispatch]);

  return (
    <MenuButtonStyle onClick={toggleSideNav}>
      <FaBars />
    </MenuButtonStyle>
  );
};

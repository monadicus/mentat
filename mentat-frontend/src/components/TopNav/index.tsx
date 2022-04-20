import React from 'react';
import styled from 'styled-components';
import { MenuButton } from './MenuButton';

export const TopNavStyle = styled.div`
  background-color: ${props => props.theme.colors.main};
  color: ${props => props.theme.colors.accent};
  height: ${props => props.theme.margin * 4}px;
  display: flex;
  align-items: center;
  padding: 0 ${props => props.theme.margin}px;
  grid-area: topnav;

  .logo {
    font-size: 24px;
    line-height: 1;
  }
`;

export const TopNav: React.FC = () => {
  return (
    <TopNavStyle>
      <MenuButton />
      <div className="logo">mentat</div>
    </TopNavStyle>
  );
};

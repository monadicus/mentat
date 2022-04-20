import React, { ReactNode } from 'react';
import styled from 'styled-components';

export const ContentStyle = styled.div`
  background-color: ${props => props.theme.colors.dominant};
  color: ${props => props.theme.colors.accent};
  position: relative;
  grid-area: content;
`;

export const Content: React.FC<{ children: ReactNode }> = ({ children }) => (
  <ContentStyle>{children}</ContentStyle>
);

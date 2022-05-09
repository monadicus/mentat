import React, { ReactNode } from 'react';
import styled from 'styled-components';

export const ContentStyle = styled.div`
  background-color: ${props => props.theme.colors.dominant};
  color: ${props => props.theme.colors.accent};
  position: relative;
  grid-area: content;
  .scroll-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
  }
`;

export const Content: React.FC<{ children: ReactNode }> = ({ children }) => (
  <ContentStyle>
    <div className="scroll-container">{children}</div>
  </ContentStyle>
);

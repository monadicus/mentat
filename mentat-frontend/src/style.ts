import { createGlobalStyle } from 'styled-components';
import media from 'styled-media-query';

export const GlobalStyle = createGlobalStyle`
  html {
    height: 100%;
  }

  body {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    background-color: ${props => props.theme.colors.dominant};
    color: ${props => props.theme.colors.accent};

    display: flex;
    align-items: stretch;
    justify-content: stretch;
    height: 100%;

    font-family: Arial, Helvetica, sans-serif;

    #app {
      display: grid;
      width: 100%;
      height: 100%;

      ${media.greaterThan('medium')`
        grid-template: "topnav topnav" auto
                     "sidenav content" 1fr / ${props =>
                       props.theme.sidenav.width}px 1fr;
      `}

      ${media.lessThan('medium')`
        grid-template: "topnav" auto
                     "content" 1fr / 1fr;
      `}
    }
  }
`;

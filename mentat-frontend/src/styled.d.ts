import 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme {
    margin: number;
    sidenav: {
      width: number;
    };
    colors: {
      main: string;
      dominant: string;
      accent: string;
      secondary: string;
    };
  }
}

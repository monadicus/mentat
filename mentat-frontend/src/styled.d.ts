import 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme {
    margin: number;
    sidenav: {
      width: number;
    };
    colors: {
      main: string;
      mainHalf: string;
      dominant: string;
      accent: string;
      accentHalf: string;
      secondary: string;
      error: string;
    };
  }
}

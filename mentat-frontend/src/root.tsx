import React from 'react';
import ReactDOM from 'react-dom/client';
import { Provider } from 'react-redux';
import { BrowserRouter } from 'react-router-dom';
import { ThemeProvider } from 'styled-components';
import { Router } from './routes';
import { store } from './store';
import { GlobalStyle } from './style';
import { theme } from './theme';

// TODO: create a nice favicon
// import '../res/favicon.ico?name=favicon.ico';

ReactDOM.createRoot(document.getElementById('app')).render(
  <ThemeProvider theme={theme}>
    <GlobalStyle />
    <Provider store={store}>
      <BrowserRouter>
        <Router />
      </BrowserRouter>
    </Provider>
  </ThemeProvider>
);

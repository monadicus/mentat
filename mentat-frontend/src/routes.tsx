import React from 'react';
import { Navigate, Outlet, Route, Routes } from 'react-router';
import { App } from './App';
import { TopNav } from './components/TopNav';
import { I18n } from './features/i18n/components';
import { useApi } from './features/rosetta/hooks';

const NotFound = () => (
  <>
    <I18n name="navigation.not_found" />
  </>
);

const Root = () => (
  <>
    <TopNav />
    <Outlet />
  </>
);

const Home = () => {
  const [status, resp] = useApi('/network/list');
  return (
    <>
      <I18n name="navigation.home" />
      <br />
      {status === 'loading' && <I18n name="navigation.loading" />}
      <pre>{JSON.stringify(resp, null, 2)}</pre>
    </>
  );
};

export const Router = () => (
  <Routes>
    <Route path="/" element={<Root />}>
      <Route index element={<Navigate replace to="/~/" />} />
      <Route path=":endpoint/" element={<App />}>
        <Route index element={<Home />} />
      </Route>
      <Route path="*" element={<NotFound />} />
    </Route>
  </Routes>
);

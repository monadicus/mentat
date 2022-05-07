import React from 'react';
import { Navigate, Route, Routes } from 'react-router';
import { App } from './App';
import { Home } from './views/home';
import { NotFound } from './views/NotFound';
import { Root } from './views/Root';

export const Router = () => (
  <Routes>
    <Route path="/" element={<Root />}>
      <Route index element={<Navigate replace to="/~/" />} />
      <Route path=":endpoint/" element={<App />}>
        <Route index element={<Home />} />
        {/* <Route path="networks" element={<NotFound />} /> */}
        <Route path="accounts" element={<NotFound />} />
        <Route path="blocks" element={<NotFound />} />
        <Route path="transactions" element={<NotFound />} />
      </Route>
      <Route path="*" element={<NotFound />} />
    </Route>
  </Routes>
);

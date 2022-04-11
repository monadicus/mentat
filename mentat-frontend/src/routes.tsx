import React from 'react';
import { Route, Routes } from 'react-router';
import { App } from './App';
import { I18n } from './features/i18n/components';

const NotFound = () => (
  <>
    <I18n name="navigation.not_found" />
  </>
);
const Home = () => (
  <>
    <I18n name="navigation.home" />
  </>
);

export const Router = () => (
  <Routes>
    <Route path="/" element={<App />}>
      <Route index element={<Home />} />
      <Route path="*" element={<NotFound />} />
    </Route>
  </Routes>
);

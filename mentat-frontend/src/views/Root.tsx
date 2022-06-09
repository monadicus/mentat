import React from 'react';
import { Outlet } from 'react-router';
import { TopNav } from '../components/TopNav';

export const Root = () => (
  <>
    <TopNav />
    <Outlet />
  </>
);

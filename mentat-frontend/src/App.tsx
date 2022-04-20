import * as React from 'react';
import { Outlet } from 'react-router';
import { Content } from './components/Content';
import { SideNav } from './components/SideNav';
import { TopNav } from './components/TopNav';

export const App = () => {
  return (
    <>
      <TopNav />
      <SideNav />
      <Content>
        <Outlet />
      </Content>
    </>
  );
};

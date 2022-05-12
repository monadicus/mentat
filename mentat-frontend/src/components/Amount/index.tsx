import React from 'react';
import { Amount } from '../../features/rosetta/models';

export const CoinAmount: React.FC<{ amount: Amount }> = ({ amount }) => {
  if (!amount) return <>unknown</>;

  const { symbol, decimals } = amount.currency;
  const sign = amount.value.includes('-') ? '-' : '';
  const value = amount.value.replace(/-/, '').padStart(decimals, '0');

  const whole = value.slice(0, -decimals) || '0';
  const fraction = value.slice(-decimals).replace(/0+$/, '');

  return (
    <span title={amount.value}>
      {sign}
      {whole}
      {fraction.length > 0 ? '.' : ''}
      {fraction} {symbol}
    </span>
  );
};

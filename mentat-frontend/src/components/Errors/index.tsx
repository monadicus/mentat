import React, { useCallback, useRef } from 'react';
import { FaExclamationCircle, FaTimes } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { dismissError, removeError } from '../../features/errors/reducer';
import {
  selectErrorById,
  selectErrorIds,
} from '../../features/errors/selectors';
import { AppDispatch } from '../../store';
import { ErrorContainerStyle, ErrorDismissButtonStyle } from './style';

const ErrorDismissButton: React.FC<{ id: string }> = ({ id }) => {
  const dispatch: AppDispatch = useDispatch();
  const clickedRef = useRef<boolean>(false);

  const dismiss = useCallback(() => {
    if (clickedRef.current) return;
    clickedRef.current = true;
    dispatch(dismissError(id));
    setTimeout(() => {
      dispatch(removeError(id));
    }, 500);
  }, [dispatch, id]);

  return (
    <ErrorDismissButtonStyle onClick={dismiss}>
      <FaTimes />
    </ErrorDismissButtonStyle>
  );
};

export const ErrorItem: React.FC<{ id: string }> = ({ id }) => {
  const error = useSelector(selectErrorById(id));
  return (
    <div className={`error-item ${error.show ? 'show' : 'hide'}`}>
      <div className="header">
        <FaExclamationCircle />
        <div className="code">[{error.code}]</div>
        <div className="message">{error.message}</div>
        <ErrorDismissButton id={id} />
      </div>
      <div className="content">
        {error.activity}
        <pre>{JSON.stringify(error.details, null, 2)}</pre>
      </div>
    </div>
  );
};

export const ErrorContainer = () => {
  const errorIds = useSelector(selectErrorIds);

  if (errorIds.length === 0) return null;

  return (
    <ErrorContainerStyle count={errorIds.length}>
      {errorIds.map(e => (
        <ErrorItem key={e} id={e} />
      ))}
    </ErrorContainerStyle>
  );
};

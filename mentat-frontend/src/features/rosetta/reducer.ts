import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import {
  NetworkIdentifier,
  NetworkOptionsResponse,
  NetworkStatusResponse,
} from './models';

type RosettaState = {
  identifiers: NetworkIdentifier[];
  network: {
    identifier: NetworkIdentifier;
    status: NetworkStatusResponse;
    options: NetworkOptionsResponse;
  };
};

const rosetta = createSlice({
  name: 'rosetta',
  initialState: {
    identifiers: [],
    network: {
      identifier: null,
      status: null,
      options: null,
    },
  } as RosettaState,
  reducers: {
    setStatus(state, action: PayloadAction<NetworkStatusResponse>) {
      state.network.status = action.payload;
    },
    setOptions(state, action: PayloadAction<NetworkOptionsResponse>) {
      state.network.options = action.payload;
    },
    setIdentifier(state, action: PayloadAction<NetworkIdentifier>) {
      state.network.identifier = action.payload;
    },
    setIdentifiers(state, action: PayloadAction<NetworkIdentifier[]>) {
      state.identifiers = action.payload;
    },
  },
});

export const { setStatus, setOptions, setIdentifier, setIdentifiers } =
  rosetta.actions;
export default rosetta.reducer;

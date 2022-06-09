export type IServer = {
  name: string;
  url: string;
};

export type IServers = Record<string, IServer>;

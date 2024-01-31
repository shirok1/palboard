type Player = {
  name: string;
  playeruid: string;
  steamid: string; // this should be considered unique
};

type UpdateSteamMessage = {
  type: "steam_self_update";
  status: string;
} | {
  type: "update_state";
  state_id: number;
  state_name: string;
  progress: string;
  current: number;
  total: number;
} | {
  type: "success"
} | {
  type: "error"; reason: string
};

import { config } from "dotenv";

const DEFAULT_CONFIG_PATH = "../.env";

config({
  path: DEFAULT_CONFIG_PATH,
  export: true,
  allowEmptyValues: true,
});

export class Config {
  DISCORD_APP_ID: string;
  DISCORD_PUB_KEY: string;
  DISCORD_BOT_TOKEN: string;
  DISCORD_BOT_URL: string;

  constructor() {
    this.DISCORD_APP_ID = mandataryEnvVar("DISCORD_APP_ID");
    this.DISCORD_PUB_KEY = mandataryEnvVar("DISCORD_PUB_KEY");
    this.DISCORD_BOT_TOKEN = mandataryEnvVar("DISCORD_BOT_TOKEN");
    this.DISCORD_BOT_URL = mandataryEnvVar("DISCORD_BOT_URL");
  }
}

function mandataryEnvVar(varName: string) {
  const value = process.env[varName];
  if (value === undefined) {
    throw new Error(
      `Missing config in ${DEFAULT_CONFIG_PATH}. ${varName} must be set`
    );
  }
  return value;
}

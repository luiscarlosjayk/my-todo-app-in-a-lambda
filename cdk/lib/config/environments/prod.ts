import type { Environment } from "../../types/environment";

const environment: Environment = {
    orgName: "luisk",
    envName: "prod",
    appName: "agro",
    region: "us-east-1",
    provisionedConcurrencyEnabled: false,
    logLevel: "debug", // TODO: Change to "info" once we are ready to go live
    domainName: "agro.luisk.dev",
};

export default environment;

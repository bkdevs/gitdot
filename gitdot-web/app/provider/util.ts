import { racePromises } from "@/util";
import { ApiProvider } from "./api";
import type { ClientProvider } from "./client";
import { DatabaseProvider } from "./database";
import type {
  ResourceDefinition,
  ResourcePromisesType,
  ResourceRequestsType,
  ResourceResultType,
} from "./types";

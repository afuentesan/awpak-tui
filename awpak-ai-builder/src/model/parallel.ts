import { Command } from "./command";
import type { DataType } from "./data";
import { DataComparatorTrue, type DataComparator } from "./data_comparator";
import { WebClient } from "./web_client";

export class Parallel
{
    executors : Array<ParallelExecutor> = [];
}

export type ParallelExecutor = ParallelExecutorCommand | ParallelExecutorWebClient;

export enum ParallelExecutorVariant
{
    Command = "Command",
    WebClient = "WebClient"
}

export class ParallelExecutorCommand
{
    readonly _variant = ParallelExecutorVariant.Command;

    ty : DataType | undefined;
    executor : Command = new Command();
    condition : DataComparator = new DataComparatorTrue();
}

export class ParallelExecutorWebClient
{
    readonly _variant = ParallelExecutorVariant.WebClient;

    ty : DataType | undefined;
    executor : WebClient = new WebClient();
    condition : DataComparator = new DataComparatorTrue();
}
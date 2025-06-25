import type { DataFrom } from "./data";

export class Command
{
    command : string | undefined;
    args : Array<DataFrom> = [];
    output : Array<CommandOutput> = [];
}

export type CommandOutput = CommandOutputOut | CommandOutputErr | CommandOutputSuccess | CommandOutputCode;

export enum CommandOutputVariant
{
    Out = "Out",
    Err = "Err",
    Success = "Success",
    Code = "Code"
}

export class CommandOutputOut
{
    readonly _variant = CommandOutputVariant.Out;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputErr
{
    readonly _variant = CommandOutputVariant.Err;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputSuccess
{
    readonly _variant = CommandOutputVariant.Success;

    prefix : string | undefined;
    suffix : string | undefined;
}

export class CommandOutputCode
{
    readonly _variant = CommandOutputVariant.Code;

    prefix : string | undefined;
    suffix : string | undefined;
}
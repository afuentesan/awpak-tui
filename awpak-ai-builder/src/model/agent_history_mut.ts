
import { FromContext, type DataFrom } from "./data";
import { DataComparatorTrue, type DataComparator } from "./data_comparator";

export class AgentHistoryMut
{
    id : string = "";
    from : DataFrom = new FromContext();
    to : DataToAgentHistory = new DataToAgentHistoryReplace();
    condition : DataComparator = new DataComparatorTrue();
}

export type DataToAgentHistory = DataToAgentHistoryReplace |
                                 DataToAgentHistoryReplaceFirst |
                                 DataToAgentHistoryReplaceLast |
                                 DataToAgentHistoryReplaceItem |
                                 DataToAgentHistoryStringToLast |
                                 DataToAgentHistoryStringToFirst |
                                 DataToAgentHistoryStringToItem;

export enum DataToAgentHistoryVariant
{
    Replace = "Replace",
    ReplaceFirst = "ReplaceFirst",
    ReplaceLast = "ReplaceLast",
    ReplaceItem = "ReplaceItem",
    StringToLast = "StringToLast",
    StringToFirst = "StringToFirst",
    StringToItem = "StringToItem"
}

export class DataToAgentHistoryReplace
{
    readonly _variant = DataToAgentHistoryVariant.Replace;
}

export class DataToAgentHistoryReplaceFirst
{
    readonly _variant = DataToAgentHistoryVariant.ReplaceFirst;
}

export class DataToAgentHistoryReplaceLast
{
    readonly _variant = DataToAgentHistoryVariant.ReplaceLast;
}

export class DataToAgentHistoryReplaceItem
{
    readonly _variant = DataToAgentHistoryVariant.ReplaceItem;

    value : number = 0;
}

export class DataToAgentHistoryStringToLast
{
    readonly _variant = DataToAgentHistoryVariant.StringToLast;
}

export class DataToAgentHistoryStringToFirst
{
    readonly _variant = DataToAgentHistoryVariant.StringToFirst;
}

export class DataToAgentHistoryStringToItem
{
    readonly _variant = DataToAgentHistoryVariant.StringToItem;

    value : number = 0;
}
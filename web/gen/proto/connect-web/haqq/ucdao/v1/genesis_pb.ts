// @generated by protoc-gen-es v1.3.1 with parameter "target=ts"
// @generated from file haqq/ucdao/v1/genesis.proto (package haqq.ucdao.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";
import { Params } from "./ucdao_pb.js";
import { Coin } from "../../../cosmos/base/v1beta1/coin_pb.js";

/**
 * GenesisState defines the gov module's genesis state.
 *
 * @generated from message haqq.ucdao.v1.GenesisState
 */
export class GenesisState extends Message<GenesisState> {
  /**
   * params defines all the parameters of the module.
   *
   * @generated from field: haqq.ucdao.v1.Params params = 1;
   */
  params?: Params;

  /**
   * balances is an array containing the balances of all the ucdao members' accounts.
   *
   * @generated from field: repeated haqq.ucdao.v1.Balance balances = 2;
   */
  balances: Balance[] = [];

  /**
   * total_balance represents the total balance of the ucdao module. If it is left empty, then supply will be calculated based on the provided
   * balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
   *
   * @generated from field: repeated cosmos.base.v1beta1.Coin total_balance = 3;
   */
  totalBalance: Coin[] = [];

  constructor(data?: PartialMessage<GenesisState>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "haqq.ucdao.v1.GenesisState";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "params", kind: "message", T: Params },
    { no: 2, name: "balances", kind: "message", T: Balance, repeated: true },
    { no: 3, name: "total_balance", kind: "message", T: Coin, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GenesisState {
    return new GenesisState().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GenesisState {
    return new GenesisState().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GenesisState {
    return new GenesisState().fromJsonString(jsonString, options);
  }

  static equals(a: GenesisState | PlainMessage<GenesisState> | undefined, b: GenesisState | PlainMessage<GenesisState> | undefined): boolean {
    return proto3.util.equals(GenesisState, a, b);
  }
}

/**
 * Balance defines an account address and balance pair used in the bank module's
 * genesis state.
 *
 * @generated from message haqq.ucdao.v1.Balance
 */
export class Balance extends Message<Balance> {
  /**
   * address is the address of the balance holder.
   *
   * @generated from field: string address = 1;
   */
  address = "";

  /**
   * coins defines the different coins this balance holds.
   *
   * @generated from field: repeated cosmos.base.v1beta1.Coin coins = 2;
   */
  coins: Coin[] = [];

  constructor(data?: PartialMessage<Balance>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "haqq.ucdao.v1.Balance";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "address", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "coins", kind: "message", T: Coin, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Balance {
    return new Balance().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Balance {
    return new Balance().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Balance {
    return new Balance().fromJsonString(jsonString, options);
  }

  static equals(a: Balance | PlainMessage<Balance> | undefined, b: Balance | PlainMessage<Balance> | undefined): boolean {
    return proto3.util.equals(Balance, a, b);
  }
}

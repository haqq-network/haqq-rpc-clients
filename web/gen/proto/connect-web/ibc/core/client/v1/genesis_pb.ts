// @generated by protoc-gen-es v1.3.1 with parameter "target=ts"
// @generated from file ibc/core/client/v1/genesis.proto (package ibc.core.client.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3, protoInt64 } from "@bufbuild/protobuf";
import { ClientConsensusStates, IdentifiedClientState, Params } from "./client_pb.js";

/**
 * GenesisState defines the ibc client submodule's genesis state.
 *
 * @generated from message ibc.core.client.v1.GenesisState
 */
export class GenesisState extends Message<GenesisState> {
  /**
   * client states with their corresponding identifiers
   *
   * @generated from field: repeated ibc.core.client.v1.IdentifiedClientState clients = 1;
   */
  clients: IdentifiedClientState[] = [];

  /**
   * consensus states from each client
   *
   * @generated from field: repeated ibc.core.client.v1.ClientConsensusStates clients_consensus = 2;
   */
  clientsConsensus: ClientConsensusStates[] = [];

  /**
   * metadata from each client
   *
   * @generated from field: repeated ibc.core.client.v1.IdentifiedGenesisMetadata clients_metadata = 3;
   */
  clientsMetadata: IdentifiedGenesisMetadata[] = [];

  /**
   * @generated from field: ibc.core.client.v1.Params params = 4;
   */
  params?: Params;

  /**
   * Deprecated: create_localhost has been deprecated.
   * The localhost client is automatically created at genesis.
   *
   * @generated from field: bool create_localhost = 5 [deprecated = true];
   * @deprecated
   */
  createLocalhost = false;

  /**
   * the sequence for the next generated client identifier
   *
   * @generated from field: uint64 next_client_sequence = 6;
   */
  nextClientSequence = protoInt64.zero;

  constructor(data?: PartialMessage<GenesisState>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "ibc.core.client.v1.GenesisState";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "clients", kind: "message", T: IdentifiedClientState, repeated: true },
    { no: 2, name: "clients_consensus", kind: "message", T: ClientConsensusStates, repeated: true },
    { no: 3, name: "clients_metadata", kind: "message", T: IdentifiedGenesisMetadata, repeated: true },
    { no: 4, name: "params", kind: "message", T: Params },
    { no: 5, name: "create_localhost", kind: "scalar", T: 8 /* ScalarType.BOOL */ },
    { no: 6, name: "next_client_sequence", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
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
 * GenesisMetadata defines the genesis type for metadata that clients may return
 * with ExportMetadata
 *
 * @generated from message ibc.core.client.v1.GenesisMetadata
 */
export class GenesisMetadata extends Message<GenesisMetadata> {
  /**
   * store key of metadata without clientID-prefix
   *
   * @generated from field: bytes key = 1;
   */
  key = new Uint8Array(0);

  /**
   * metadata value
   *
   * @generated from field: bytes value = 2;
   */
  value = new Uint8Array(0);

  constructor(data?: PartialMessage<GenesisMetadata>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "ibc.core.client.v1.GenesisMetadata";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "key", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
    { no: 2, name: "value", kind: "scalar", T: 12 /* ScalarType.BYTES */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GenesisMetadata {
    return new GenesisMetadata().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GenesisMetadata {
    return new GenesisMetadata().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GenesisMetadata {
    return new GenesisMetadata().fromJsonString(jsonString, options);
  }

  static equals(a: GenesisMetadata | PlainMessage<GenesisMetadata> | undefined, b: GenesisMetadata | PlainMessage<GenesisMetadata> | undefined): boolean {
    return proto3.util.equals(GenesisMetadata, a, b);
  }
}

/**
 * IdentifiedGenesisMetadata has the client metadata with the corresponding
 * client id.
 *
 * @generated from message ibc.core.client.v1.IdentifiedGenesisMetadata
 */
export class IdentifiedGenesisMetadata extends Message<IdentifiedGenesisMetadata> {
  /**
   * @generated from field: string client_id = 1;
   */
  clientId = "";

  /**
   * @generated from field: repeated ibc.core.client.v1.GenesisMetadata client_metadata = 2;
   */
  clientMetadata: GenesisMetadata[] = [];

  constructor(data?: PartialMessage<IdentifiedGenesisMetadata>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "ibc.core.client.v1.IdentifiedGenesisMetadata";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "client_id", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "client_metadata", kind: "message", T: GenesisMetadata, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): IdentifiedGenesisMetadata {
    return new IdentifiedGenesisMetadata().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): IdentifiedGenesisMetadata {
    return new IdentifiedGenesisMetadata().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): IdentifiedGenesisMetadata {
    return new IdentifiedGenesisMetadata().fromJsonString(jsonString, options);
  }

  static equals(a: IdentifiedGenesisMetadata | PlainMessage<IdentifiedGenesisMetadata> | undefined, b: IdentifiedGenesisMetadata | PlainMessage<IdentifiedGenesisMetadata> | undefined): boolean {
    return proto3.util.equals(IdentifiedGenesisMetadata, a, b);
  }
}

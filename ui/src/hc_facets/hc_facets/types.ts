import type { 
  Record, 
  ActionHash,
  DnaHash,
  SignedActionHashed,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type HcFacetsSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'FacetValue'; } & FacetValue)
 | ({ type: 'FacetOption'; } & FacetOption)
 | ({  type: 'FacetGroup'; } & FacetGroup);



export interface FacetGroup { 
  name: string;

  note: string;
}




export interface FacetOption { 
  name: string;

  note: string;

  facetGroupId: ActionHash | undefined;
}




export interface FacetValue { 
  facet_value: string;

  facet_option: ActionHash;

  record_type: string | undefined;
}


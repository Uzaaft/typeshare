/*
 Generated by typeshare 1.2.0
*/

export interface AddressDetails {
}

export type Address = 
	| { type: "FixedAddress", content: AddressDetails }
	| { type: "NoFixedAddress", content?: undefined };


import Base from "./base";
import Any from "./any";

export interface UUID {
    type: 'uuid';
}

export interface Address {
    type: 'address';
}

export interface Email {
    type: 'email';
    firstName?: string;
    lastName?: string;
    domain?: string;
}

export interface FirstName {
    type: 'firstName';
}

export interface LastName {
    type: 'lastName';
}

export interface FullName {
    type: 'fullName';
}

export interface CompanyName {
    type: 'companyName';
}

export interface Website {
    type: 'website';
}

export interface PhoneNumber {
    type: 'phoneNumber';
}

export interface Country {
    type: 'country';
}

export interface City {
    type: 'city';
}

export interface ZipCode {
    type: 'zipCode';
}

export interface Latitude {
    type: 'latitude';
}

export interface Longitude {
    type: 'longitude';
}

export interface Color {
    type: 'color';
}

export interface Title {
    type: 'title';
}

export interface Username {
    type: 'username';
    firstName?: string;
    lastName?: string;
}

export interface Format {
    type: 'format';
    format: string;
    args: Any[];
}

export interface Password {
    type: 'password';
    length?: number;
}

type StringGenerator = UUID | Address | Email | FirstName | LastName | FullName | CompanyName | Website | PhoneNumber | Country | City | ZipCode | Latitude | Longitude | Color | Title | Username | Format | Password;

export interface GeneratedString extends Base {
    type: 'string';
    generator: StringGenerator;
}

export interface ConstantString extends Base {
    type: 'string';
    value: string;
}

type String = GeneratedString | ConstantString;
export default String;

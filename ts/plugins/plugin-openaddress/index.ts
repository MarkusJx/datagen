import { GenerateFn } from '../../base/plugin';
import * as addresses from './us_ak_haines.json';

type Address = {
    street: string;
    number: string;
    city: string;
    longitude: number;
    latitude: number;
    country: string;
}

export const generate: GenerateFn<{}, Address> = () => {
    const address = addresses[Math.floor(Math.random() * addresses.length)];
    return {
        street: address.properties.street,
        number: address.properties.number,
        city: address.properties.city,
        longitude: address.geometry.coordinates[0],
        latitude: address.geometry.coordinates[1],
        country: 'United States of America'
    }
}

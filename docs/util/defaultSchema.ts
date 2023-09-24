import { Schema } from '@datagen-rs/types';

const defaultSchema: Schema = {
  options: {
    serializer: {
      type: 'json',
      pretty: true,
    },
  },
  type: 'array',
  length: {
    min: 1,
    max: 5,
  },
  items: {
    type: 'object',
    properties: {
      id: {
        type: 'string',
        generator: {
          type: 'uuid',
        },
      },
      firstName: {
        type: 'string',
        generator: {
          type: 'firstName',
        },
      },
      lastName: {
        type: 'string',
        generator: {
          type: 'lastName',
        },
      },
      fullName: {
        type: 'string',
        generator: {
          type: 'format',
          format: '{{firstName}} {{lastName}}',
          args: {
            firstName: 'ref:./firstName',
            lastName: 'ref:./lastName',
          },
        },
      },
      age: {
        type: 'integer',
        min: 18,
        max: 99,
      },
      email: {
        type: 'string',
        generator: {
          type: 'email',
        },
      },
      address: {
        type: 'object',
        properties: {
          street: {
            type: 'string',
            generator: {
              type: 'street',
            },
          },
          city: {
            type: 'string',
            generator: {
              type: 'city',
            },
          },
          zip: {
            type: 'string',
            generator: {
              type: 'zipCode',
            },
          },
          state: {
            type: 'string',
            generator: {
              type: 'state',
            },
          },
        },
      },
    },
  },
};

export default JSON.stringify(defaultSchema, null, 2);

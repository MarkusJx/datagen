import React from 'react';
import { Callout } from 'nextra/components';
import { isMobile } from 'react-device-detect';
import DemoElements from './DemoElements';
import { useMediaQuery } from '@mui/material';
import Center from './Center';

const Demo: React.FC = () => {
  const tooSmall = useMediaQuery('(max-width: 1000px)');

  if (isMobile) {
    return (
      <Callout type="warning">
        The demo is not available on mobile devices.
      </Callout>
    );
  } else if (tooSmall) {
    return (
      <Callout type="warning">
        The demo is not available on devices with a screen width of less than
        1000px.
      </Callout>
    );
  } else {
    return (
      <Center>
        <DemoElements />
      </Center>
    );
  }
};

export default Demo;

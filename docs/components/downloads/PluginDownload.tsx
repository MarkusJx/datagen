import React from 'react';
import { Button } from '@mui/material';
import Center from '../Center';
import OSIcon from './OSIcon';

interface Props {
  name: string;
  url: string;
}

const PluginDownload: React.FC<Props> = ({ name, url }) => {
  return (
    <Center>
      <Button
        startIcon={<OSIcon />}
        variant="contained"
        href={url}
        sx={{
          margin: '2rem 0',
          textTransform: 'none',
        }}
      >
        Download {name}
      </Button>
    </Center>
  );
};

export default PluginDownload;

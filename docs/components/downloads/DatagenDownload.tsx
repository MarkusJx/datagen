import React from 'react';
import { Button } from '@mui/material';
import OSIcon from './OSIcon';
import Center from '../Center';
import { useDownloadsContext } from './DownloadsContext';

interface Props {
  children: React.ReactNode;
}

const DatagenDownload: React.FC<Props> = ({ children }) => {
  const { osArtifacts } = useDownloadsContext();

  return (
    <div>
      <div>{children}</div>
      <Center>
        <Button
          startIcon={<OSIcon />}
          variant="contained"
          href={osArtifacts?.['datagen']}
          sx={{ marginTop: '2rem', textTransform: 'none' }}
        >
          Download datagen
        </Button>
      </Center>
    </div>
  );
};

export default DatagenDownload;

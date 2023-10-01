import React from 'react';
import { SxProps, Theme } from '@mui/material';
import StyledDiv from '../styled/StyledDiv';

interface Props {
  children?: React.ReactNode;
  maxWidth?: string;
  sx?: SxProps<Theme>;
}

const DemoGrid: React.FC<Props> = ({ children, maxWidth, sx }) => {
  return (
    <StyledDiv
      sx={{
        maxWidth,
        ...sx,
      }}
    >
      {children}
    </StyledDiv>
  );
};

export default DemoGrid;

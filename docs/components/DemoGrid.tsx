import React from 'react';
import { styled, SxProps, Theme } from '@mui/material';

interface Props {
  children?: React.ReactNode;
  maxWidth?: string;
  sx?: SxProps<Theme>;
}

const StyledDiv = styled('div')`
  display: grid;
  row-gap: 2rem;
`;

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

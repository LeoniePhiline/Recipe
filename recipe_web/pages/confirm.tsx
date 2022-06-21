import { Box, Heading, Link, Text } from '@chakra-ui/react';
import { CheckCircleIcon } from '@chakra-ui/icons';
import NextLink from 'next/link';
import { useAuth } from '../utils/useAuth';

export default function Confirm() {
  return (
    <Box textAlign="center" py={10} px={6}>
      <CheckCircleIcon boxSize={'50px'} color={'green.500'} />
      <Heading as="h2" size="xl" mt={6} mb={2}>
        All set.
      </Heading>
      <Text color={'gray.500'}>
        We have sent you an activation email. You can now{' '}
        <NextLink href="/login">
          <Link color={'orange.400'}>login.</Link>
        </NextLink>
      </Text>
    </Box>
  );
}

import React, { useState } from 'react';
import {
  ChakraProvider,
  Box,
  VStack,
  Heading,
  Text,
  Input,
  Button,
  useToast,
  Container,
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  FormControl,
  FormLabel,
  NumberInput,
  NumberInputField,
  NumberInputStepper,
  NumberIncrementStepper,
  NumberDecrementStepper,
} from '@chakra-ui/react';
import { Horizon } from '@stellar/stellar-sdk';
import * as Contract from './contract';

const CONTRACT_ADDRESS = 'CCQWMCHX6GPZDCKZRACJM35FRSA5M6BNG5Q23GRT4IPGG7I6P5ZAURV4';
const DEFAULT_ACCOUNT = 'GDOBGW2B4PHXXEOWAACCEVDZJJ6Q2EUZGTK5ZQNEVZZFP4VC3MD4223B';

function App() {
  const [publicKey, setPublicKey] = useState(DEFAULT_ACCOUNT);
  const [loading, setLoading] = useState(false);
  const [balance, setBalance] = useState(null);
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState(0);
  const [spender, setSpender] = useState('');
  const [allowance, setAllowance] = useState(0);
  const [from, setFrom] = useState('');
  const [burnAmount, setBurnAmount] = useState(0);
  const [burnFromSpender, setBurnFromSpender] = useState('');
  const [burnFromFrom, setBurnFromFrom] = useState('');
  const [burnFromAmount, setBurnFromAmount] = useState(0);
  const [freezeAccountAddr, setFreezeAccountAddr] = useState('');
  const [unfreezeAccountAddr, setUnfreezeAccountAddr] = useState('');
  const [multisigOwners, setMultisigOwners] = useState('');
  const [multisigThreshold, setMultisigThreshold] = useState(2);
  const [msFunction, setMsFunction] = useState('transfer');
  const [msTo, setMsTo] = useState('');
  const [msAmount, setMsAmount] = useState(0);
  const [msExpiration, setMsExpiration] = useState(0);
  const [msTxId, setMsTxId] = useState('');
  const toast = useToast();

  const handleConnect = async () => {
    try {
      setLoading(true);
      const server = new Horizon.Server('https://horizon-testnet.stellar.org');
      const account = await server.loadAccount(publicKey);
      setBalance(account.balances[0]?.balance || '0');
      
      toast({
        title: 'Connected successfully',
        description: `Connected to account: ${publicKey}`,
        status: 'success',
        duration: 5000,
        isClosable: true,
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: error.message,
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleTransfer = async () => {
    try {
      setLoading(true);
      const transaction = await Contract.transfer(publicKey, recipient, amount);
      
      toast({
        title: 'Transfer initiated',
        description: `Transferring ${amount} tokens to ${recipient}`,
        status: 'info',
        duration: 5000,
        isClosable: true,
      });
    } catch (error) {
      toast({
        title: 'Transfer failed',
        description: error.message,
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleApprove = async () => {
    try {
      setLoading(true);
      const transaction = await Contract.approve(publicKey, spender, allowance);
      
      toast({
        title: 'Approval initiated',
        description: `Approving ${allowance} tokens for ${spender}`,
        status: 'info',
        duration: 5000,
        isClosable: true,
      });
    } catch (error) {
      toast({
        title: 'Approval failed',
        description: error.message,
        status: 'error',
        duration: 5000,
        isClosable: true,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleTransferFrom = async () => {
    try {
      setLoading(true);
      await Contract.transferFrom(spender, from, recipient, amount);
      toast({ title: 'TransferFrom çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'TransferFrom başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleBurn = async () => {
    try {
      setLoading(true);
      await Contract.burn(publicKey, burnAmount);
      toast({ title: 'Burn çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'Burn başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleBurnFrom = async () => {
    try {
      setLoading(true);
      await Contract.burnFrom(burnFromSpender, burnFromFrom, burnFromAmount);
      toast({ title: 'BurnFrom çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'BurnFrom başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleFreeze = async () => {
    try {
      setLoading(true);
      await Contract.freezeAccount(publicKey, freezeAccountAddr);
      toast({ title: 'FreezeAccount çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'FreezeAccount başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleUnfreeze = async () => {
    try {
      setLoading(true);
      await Contract.unfreezeAccount(publicKey, unfreezeAccountAddr);
      toast({ title: 'UnfreezeAccount çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'UnfreezeAccount başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleSetupMultisig = async () => {
    try {
      setLoading(true);
      const ownersArr = multisigOwners.split(',').map(s => s.trim());
      await Contract.setupMultisig(publicKey, ownersArr, multisigThreshold);
      toast({ title: 'setupMultisig çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'setupMultisig başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleProposeMultisig = async () => {
    try {
      setLoading(true);
      await Contract.proposeMultisigTransaction(publicKey, msFunction, msTo, msAmount, msExpiration);
      toast({ title: 'proposeMultisigTransaction çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'proposeMultisigTransaction başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  const handleApproveMultisig = async () => {
    try {
      setLoading(true);
      await Contract.approveMultisigTransaction(publicKey, msTxId);
      toast({ title: 'approveMultisigTransaction çağrıldı', status: 'info', duration: 5000, isClosable: true });
    } catch (error) {
      toast({ title: 'approveMultisigTransaction başarısız', description: error.message, status: 'error', duration: 5000, isClosable: true });
    } finally {
      setLoading(false);
    }
  };

  return (
    <ChakraProvider>
      <Box minH="100vh" bg="gray.50" py={10}>
        <Container maxW="container.md">
          <VStack spacing={8} align="stretch">
            <Heading textAlign="center" color="blue.600">
              MetaSigChain
            </Heading>
            <Text textAlign="center" color="gray.600">
              Smart Contract Address: {CONTRACT_ADDRESS}
            </Text>
            
            <Box bg="white" p={8} borderRadius="lg" boxShadow="md">
              <VStack spacing={4}>
                <Input
                  placeholder="Enter your Stellar public key"
                  value={publicKey}
                  onChange={(e) => setPublicKey(e.target.value)}
                />
                <Button
                  colorScheme="blue"
                  width="full"
                  onClick={handleConnect}
                  isLoading={loading}
                >
                  Connect Wallet
                </Button>
                {balance && (
                  <Text mt={4} color="gray.600">
                    Account Balance: {balance} XLM
                  </Text>
                )}
              </VStack>
            </Box>

            {balance && (
              <Box bg="white" p={8} borderRadius="lg" boxShadow="md">
                <Tabs>
                  <TabList>
                    <Tab>Transfer</Tab>
                    <Tab>Approve</Tab>
                    <Tab>Transfer From</Tab>
                    <Tab>Burn</Tab>
                    <Tab>Burn From</Tab>
                    <Tab>Freeze</Tab>
                    <Tab>Unfreeze</Tab>
                    <Tab>Setup Multisig</Tab>
                    <Tab>Propose Multisig</Tab>
                    <Tab>Approve Multisig</Tab>
                  </TabList>

                  <TabPanels>
                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Recipient Address</FormLabel>
                          <Input
                            value={recipient}
                            onChange={(e) => setRecipient(e.target.value)}
                            placeholder="Enter recipient address"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Amount</FormLabel>
                          <NumberInput
                            value={amount}
                            onChange={(value) => setAmount(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleTransfer}
                          isLoading={loading}
                        >
                          Transfer
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Spender Address</FormLabel>
                          <Input
                            value={spender}
                            onChange={(e) => setSpender(e.target.value)}
                            placeholder="Enter spender address"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Allowance Amount</FormLabel>
                          <NumberInput
                            value={allowance}
                            onChange={(value) => setAllowance(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleApprove}
                          isLoading={loading}
                        >
                          Approve
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>From Address</FormLabel>
                          <Input
                            value={from}
                            onChange={(e) => setFrom(e.target.value)}
                            placeholder="Enter from address"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Amount</FormLabel>
                          <NumberInput
                            value={amount}
                            onChange={(value) => setAmount(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleTransferFrom}
                          isLoading={loading}
                        >
                          Transfer From
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Burn Amount</FormLabel>
                          <NumberInput
                            value={burnAmount}
                            onChange={(value) => setBurnAmount(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleBurn}
                          isLoading={loading}
                        >
                          Burn
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Burn From Spender</FormLabel>
                          <Input
                            value={burnFromSpender}
                            onChange={(e) => setBurnFromSpender(e.target.value)}
                            placeholder="Enter burn from spender"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Burn From From</FormLabel>
                          <Input
                            value={burnFromFrom}
                            onChange={(e) => setBurnFromFrom(e.target.value)}
                            placeholder="Enter burn from from"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Burn From Amount</FormLabel>
                          <NumberInput
                            value={burnFromAmount}
                            onChange={(value) => setBurnFromAmount(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleBurnFrom}
                          isLoading={loading}
                        >
                          Burn From
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Freeze Account Address</FormLabel>
                          <Input
                            value={freezeAccountAddr}
                            onChange={(e) => setFreezeAccountAddr(e.target.value)}
                            placeholder="Enter freeze account address"
                          />
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleFreeze}
                          isLoading={loading}
                        >
                          Freeze
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Unfreeze Account Address</FormLabel>
                          <Input
                            value={unfreezeAccountAddr}
                            onChange={(e) => setUnfreezeAccountAddr(e.target.value)}
                            placeholder="Enter unfreeze account address"
                          />
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleUnfreeze}
                          isLoading={loading}
                        >
                          Unfreeze
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Multisig Owners</FormLabel>
                          <Input
                            value={multisigOwners}
                            onChange={(e) => setMultisigOwners(e.target.value)}
                            placeholder="Enter multisig owners separated by commas"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Multisig Threshold</FormLabel>
                          <NumberInput
                            value={multisigThreshold}
                            onChange={(value) => setMultisigThreshold(value)}
                            min={1}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleSetupMultisig}
                          isLoading={loading}
                        >
                          Setup Multisig
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Multisig Function</FormLabel>
                          <Input
                            value={msFunction}
                            onChange={(e) => setMsFunction(e.target.value)}
                            placeholder="Enter multisig function"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Multisig To</FormLabel>
                          <Input
                            value={msTo}
                            onChange={(e) => setMsTo(e.target.value)}
                            placeholder="Enter multisig to"
                          />
                        </FormControl>
                        <FormControl>
                          <FormLabel>Multisig Amount</FormLabel>
                          <NumberInput
                            value={msAmount}
                            onChange={(value) => setMsAmount(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <FormControl>
                          <FormLabel>Multisig Expiration</FormLabel>
                          <NumberInput
                            value={msExpiration}
                            onChange={(value) => setMsExpiration(value)}
                            min={0}
                          >
                            <NumberInputField />
                            <NumberInputStepper>
                              <NumberIncrementStepper />
                              <NumberDecrementStepper />
                            </NumberInputStepper>
                          </NumberInput>
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleProposeMultisig}
                          isLoading={loading}
                        >
                          Propose Multisig
                        </Button>
                      </VStack>
                    </TabPanel>

                    <TabPanel>
                      <VStack spacing={4}>
                        <FormControl>
                          <FormLabel>Multisig Transaction ID</FormLabel>
                          <Input
                            value={msTxId}
                            onChange={(e) => setMsTxId(e.target.value)}
                            placeholder="Enter multisig transaction ID"
                          />
                        </FormControl>
                        <Button
                          colorScheme="blue"
                          width="full"
                          onClick={handleApproveMultisig}
                          isLoading={loading}
                        >
                          Approve Multisig
                        </Button>
                      </VStack>
                    </TabPanel>
                  </TabPanels>
                </Tabs>
              </Box>
            )}
          </VStack>
        </Container>
      </Box>
    </ChakraProvider>
  );
}

export default App; 
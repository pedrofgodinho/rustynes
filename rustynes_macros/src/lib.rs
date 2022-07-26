#[cfg(test)]
mod test;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{braced, Expr, parenthesized, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

/// $TO_MATCH { $($INSTRUCTION: $($OPCODE => MODE ($BYTES)),+)+; }
struct InstructionSetCallMatch {
    to_match: Ident,
    instructions: InstructionSet,
}

struct InstructionSetDisassemblyMatch {
    operands: Ident,
    address: Ident,
    instructions: InstructionSet,
}

struct InstructionSet {
    instructions: Punctuated<Instruction, Token![;]>,
}

struct Instruction {
    instruction: Ident,
    opcodes: Punctuated<Opcode, Token![,]>,
}

struct Opcode {
    opcode: Expr,
    mode: Ident,
    bytes: Expr,
}

impl Parse for InstructionSetCallMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let to_match = input.parse()?;
        let content;
        braced!(content in input);
        let instructions = content.parse()?;
        Ok(InstructionSetCallMatch {
            to_match,
            instructions,
        })
    }
}

impl Parse for InstructionSetDisassemblyMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let address = input.parse()?;
        input.parse::<Token![,]>()?;
        let operands= input.parse()?;
        let content;
        braced!(content in input);
        let instructions = content.parse()?;
        Ok(InstructionSetDisassemblyMatch {
            operands,
            address,
            instructions,
        })
    }
}

impl Parse for InstructionSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let instructions = Punctuated::<Instruction, Token![;]>::parse_terminated(input)?;
        Ok(InstructionSet { instructions })
    }
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let instruction = input.parse()?;
        input.parse::<Token![:]>()?;
        let opcodes = Punctuated::<Opcode, Token![,]>::parse_separated_nonempty(input)?;
        Ok(Instruction {
            instruction,
            opcodes,
        })
    }

}

impl Parse for Opcode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let opcode = input.parse()?;
        input.parse::<Token![=>]>()?;
        let mode = input.parse()?;
        let content;
        parenthesized!(content in input);
        let bytes = content.parse()?;

        Ok(Opcode {
            opcode,
            mode,
            bytes,
        })
    }
}

impl Instruction {
    fn operation_match_arms(&self) -> TokenStream {
        let instruction = &self.instruction;
        let opcodes = &self.opcodes;

        let mut output = TokenStream::new();
        for opcode in opcodes {
            let mode = &opcode.mode;
            let bytes = &opcode.bytes;
            let opcode = &opcode.opcode;
            output.extend(quote!{
                #opcode => {
                    let op_result = self.#instruction(AddressingMode::#mode)?;
                    if op_result.increment_pc {
                        self.register_pc = self.register_pc.wrapping_add(#bytes - 1);
                    }
                    Ok(op_result.extra_cycles)
                },
            });
        }

        output
    }

    fn disassembly_match_arms(&self) -> TokenStream {
        let instruction = &self.instruction;
        let opcodes = &self.opcodes;

        let mut output = TokenStream::new();
        for opcode in opcodes {
            let mode = &opcode.mode;
            let bytes = &opcode.bytes;
            let opcode = &opcode.opcode;
            output.extend(quote!{
                #opcode => {
                    Ok(Instruction {
                        opcode: operands[0],
                        operands: {vec![operands[1], operands[2]]},
                        address,
                        instruction: stringify!(#instruction),
                        addressing_mode: AddressingMode::#mode,
                        length: #bytes,
                    }
                    )
                },
            });
        }

        output
    }
}

#[proc_macro]
pub fn call_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = call_op2(TokenStream::from(input));
    proc_macro::TokenStream::from(output)
}

fn call_op2(input: TokenStream) -> TokenStream {
    let InstructionSetCallMatch {
        to_match,
        instructions: InstructionSet { instructions },
    } = syn::parse2(input).unwrap();

    let mut match_arms = TokenStream::new();

    for instruction in instructions {
        match_arms.extend(instruction.operation_match_arms());
    }

    let output = quote! {
        match #to_match {
            #match_arms
            _ => Err(EmulationError::InvalidOpcode(opcode)),
        }
    };

    output
}

#[proc_macro]
pub fn disassemble_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = disassemble_op2(TokenStream::from(input));
    proc_macro::TokenStream::from(output)
}

fn disassemble_op2(input: TokenStream) -> TokenStream {
    let InstructionSetDisassemblyMatch {
        operands,
        address,
        instructions: InstructionSet { instructions },
    } = syn::parse2(input).unwrap();

    let mut match_arms = TokenStream::new();

    for instruction in instructions {
        match_arms.extend(instruction.disassembly_match_arms());
    }

    let output = quote! {
        {
            let operands = #operands;
            let address = #address;
            match operands[0] {
                #match_arms
                _ => Err(EmulationError::InvalidOpcode(operands[0])),
            }
        }
    };

    output
}


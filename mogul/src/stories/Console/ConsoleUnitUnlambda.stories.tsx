import type { Meta, StoryObj } from "storybook-solidjs";
import { ConsoleUnitUnlambda } from "~/components/Console";

const meta = {
  component: ConsoleUnitUnlambda,
  parameters: {
    layout: "fullscreen",
  },
} satisfies Meta<typeof ConsoleUnitUnlambda>;

export default meta;
type Story = StoryObj<typeof meta>;

export const LazyK: Story = {
  name: "Lazy_K style",
  args: {
    displayStyle: "LazyK",
    expr: dummyExpr(),
    result: dummyResult(),
  },
};

export const EcmaScript: Story = {
  name: "ECMAScript style",
  args: {
    displayStyle: "EcmaScript",
    expr: dummyExpr(),
    result: dummyResult(),
  },
};

// ========================================================================== //

function dummyExpr() {
  return {
    Lambda: {
      param: "f",
      body: {
        Lambda: {
          param: "x",
          body: {
            Lambda: {
              param: "y",
              body: {
                Apply: {
                  lhs: {
                    Apply: {
                      lhs: {
                        Variable: {
                          identifier: "f",
                        },
                      },
                      rhs: {
                        Variable: {
                          identifier: "y",
                        },
                      },
                    },
                  },
                  rhs: {
                    Variable: {
                      identifier: "x",
                    },
                  },
                },
              },
            },
          },
        },
      },
    },
  };
}

function dummyResult() {
  return {
    Apply: {
      lhs: {
        Apply: {
          lhs: {
            Variable: {
              identifier: "s",
            },
          },
          rhs: {
            Apply: {
              lhs: {
                Apply: {
                  lhs: {
                    Variable: {
                      identifier: "s",
                    },
                  },
                  rhs: {
                    Apply: {
                      lhs: {
                        Variable: {
                          identifier: "k",
                        },
                      },
                      rhs: {
                        Variable: {
                          identifier: "s",
                        },
                      },
                    },
                  },
                },
              },
              rhs: {
                Apply: {
                  lhs: {
                    Apply: {
                      lhs: {
                        Variable: {
                          identifier: "s",
                        },
                      },
                      rhs: {
                        Apply: {
                          lhs: {
                            Variable: {
                              identifier: "k",
                            },
                          },
                          rhs: {
                            Variable: {
                              identifier: "k",
                            },
                          },
                        },
                      },
                    },
                  },
                  rhs: {
                    Variable: {
                      identifier: "s",
                    },
                  },
                },
              },
            },
          },
        },
      },
      rhs: {
        Apply: {
          lhs: {
            Variable: {
              identifier: "k",
            },
          },
          rhs: {
            Variable: {
              identifier: "k",
            },
          },
        },
      },
    },
  };
}
